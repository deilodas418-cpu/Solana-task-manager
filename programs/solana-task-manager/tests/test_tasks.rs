use {
    anchor_lang::{
        prelude::Pubkey,
        solana_program::{instruction::Instruction, system_program},
        AccountDeserialize, InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

fn program_bytes() -> Vec<u8> {
    include_bytes!(concat!(
        env!("CARGO_TARGET_TMPDIR"),
        "/../deploy/solana_task_manager.so"
    ))
    .to_vec()
}

fn setup() -> (LiteSVM, Keypair, Pubkey, Pubkey) {
    let program_id = solana_task_manager::id();
    let payer = Keypair::new();
    let (profile, _) = Pubkey::find_program_address(
        &[solana_task_manager::constants::PROFILE_SEED, payer.pubkey().as_ref()],
        &program_id,
    );
    let mut svm = LiteSVM::new();
    svm.add_program(program_id, &program_bytes()).unwrap();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();
    (svm, payer, profile, program_id)
}

fn send(
    svm: &mut LiteSVM,
    payer: &Keypair,
    ix: Instruction,
) -> Result<(), String> {
    // LiteSVM no avanza el blockhash solo: sin esto, dos DeleteTask
    // idénticos (el rechazado y el válido) tendrían la misma firma y
    // el segundo fallaría con AlreadyProcessed aunque el programa lo acepte.
    svm.expire_blockhash();
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer])
        .map_err(|e| e.to_string())?;
    svm.send_transaction(tx).map(|_| ()).map_err(|e| format!("{e:?}"))
}

fn send_with_signer(
    svm: &mut LiteSVM,
    payer: &Keypair,
    signer: &Keypair,
    ix: Instruction,
) -> Result<(), String> {
    // payer pays fees, signer is the instruction authority
    svm.expire_blockhash();
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer, signer])
        .map_err(|e| e.to_string())?;
    svm.send_transaction(tx).map(|_| ()).map_err(|e| format!("{e:?}"))
}

fn task_pda(creator: &Pubkey, id: u64, program_id: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[
            solana_task_manager::constants::TASK_SEED,
            creator.as_ref(),
            id.to_le_bytes().as_ref(),
        ],
        program_id,
    )
    .0
}

fn init_profile(svm: &mut LiteSVM, payer: &Keypair, profile: Pubkey, program_id: Pubkey) {
    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_task_manager::instruction::InitializeProfile {}.data(),
        solana_task_manager::accounts::InitializeProfile {
            owner: payer.pubkey(),
            profile,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    send(svm, payer, ix).expect("initialize_profile debe funcionar");
}

fn create_task(
    svm: &mut LiteSVM,
    payer: &Keypair,
    profile: Pubkey,
    task: Pubkey,
    program_id: Pubkey,
    title: &str,
    description: &str,
) {
    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_task_manager::instruction::CreateTask {
            title: title.to_string(),
            description: description.to_string(),
        }
        .data(),
        solana_task_manager::accounts::CreateTask {
            creator: payer.pubkey(),
            profile,
            task,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    send(svm, payer, ix).expect("create_task debe funcionar");
}

fn read_task(svm: &LiteSVM, task: Pubkey) -> solana_task_manager::state::Task {
    let acc = svm.get_account(&task).expect("la tarea debe existir");
    let mut data: &[u8] = &acc.data;
    solana_task_manager::state::Task::try_deserialize(&mut data).unwrap()
}

fn read_profile(svm: &LiteSVM, profile: Pubkey) -> solana_task_manager::state::Profile {
    let acc = svm.get_account(&profile).expect("el perfil debe existir");
    let mut data: &[u8] = &acc.data;
    solana_task_manager::state::Profile::try_deserialize(&mut data).unwrap()
}

#[test]
fn test_flujo_completo_crear_leer_actualizar_completar_y_cerrar() {
    let (mut svm, payer, profile, program_id) = setup();
    init_profile(&mut svm, &payer, profile, program_id);

    let p = read_profile(&svm, profile);
    assert_eq!(p.owner, payer.pubkey());
    assert_eq!(p.task_count, 0);

    // Crear tarea 0
    let task0 = task_pda(&payer.pubkey(), 0, &program_id);
    create_task(
        &mut svm, &payer, profile, task0, program_id,
        "Comprar SOL para pruebas", "Airdrop en devnet y probar el flujo",
    );
    assert_eq!(read_profile(&svm, profile).task_count, 1);
    let t = read_task(&svm, task0);
    assert_eq!(t.creator, payer.pubkey());
    assert_eq!(t.id, 0);
    assert_eq!(t.title, "Comprar SOL para pruebas");
    assert!(!t.completed);

    // Actualizar (solo creador, solo si no está completada)
    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_task_manager::instruction::UpdateTask {
            title: "Comprar SOL (actualizado)".to_string(),
            description: "Descripción actualizada".to_string(),
        }
        .data(),
        solana_task_manager::accounts::UpdateTask {
            creator: payer.pubkey(),
            task: task0,
        }
        .to_account_metas(None),
    );
    send(&mut svm, &payer, ix).expect("update_task debe funcionar");
    assert_eq!(read_task(&svm, task0).title, "Comprar SOL (actualizado)");

    // Intentar borrar antes de completar: debe RECHAZARSE (TaskNotCompleted)
    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_task_manager::instruction::DeleteTask {}.data(),
        solana_task_manager::accounts::DeleteTask {
            creator: payer.pubkey(),
            task: task0,
        }
        .to_account_metas(None),
    );
    let err = send(&mut svm, &payer, ix).expect_err("delete antes de completar debe fallar");
    assert!(err.contains("TaskNotCompleted"), "error esperado TaskNotCompleted, got: {err}");
    assert!(svm.get_account(&task0).is_some(), "la tarea no debe haberse cerrado");

    // Completar
    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_task_manager::instruction::CompleteTask {}.data(),
        solana_task_manager::accounts::CompleteTask {
            creator: payer.pubkey(),
            task: task0,
        }
        .to_account_metas(None),
    );
    send(&mut svm, &payer, ix).expect("complete_task debe funcionar");
    assert!(read_task(&svm, task0).completed);

    // Actualizar una tarea completada: debe rechazarse
    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_task_manager::instruction::UpdateTask {
            title: "No debería".to_string(),
            description: "x".to_string(),
        }
        .data(),
        solana_task_manager::accounts::UpdateTask {
            creator: payer.pubkey(),
            task: task0,
        }
        .to_account_metas(None),
    );
    assert!(send(&mut svm, &payer, ix).is_err(), "update de tarea completada debe fallar");

    // Ahora sí borrar: cierra la cuenta y devuelve la renta
    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_task_manager::instruction::DeleteTask {}.data(),
        solana_task_manager::accounts::DeleteTask {
            creator: payer.pubkey(),
            task: task0,
        }
        .to_account_metas(None),
    );
    send(&mut svm, &payer, ix).expect("delete tras completar debe funcionar");
    assert!(svm.get_account(&task0).is_none(), "la cuenta debe estar cerrada");
}

#[test]
fn test_permisos_otro_usuario_no_puede_modificar() {
    let (mut svm, payer, profile, program_id) = setup();
    init_profile(&mut svm, &payer, profile, program_id);
    let task0 = task_pda(&payer.pubkey(), 0, &program_id);
    create_task(&mut svm, &payer, profile, task0, program_id, "Tarea propia", "desc");

    let intruso = Keypair::new();
    svm.airdrop(&intruso.pubkey(), 1_000_000_000).unwrap();

    // update por intruso
    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_task_manager::instruction::UpdateTask {
            title: "Hack".to_string(),
            description: "x".to_string(),
        }
        .data(),
        solana_task_manager::accounts::UpdateTask {
            creator: intruso.pubkey(),
            task: task0,
        }
        .to_account_metas(None),
    );
    assert!(send_with_signer(&mut svm, &payer, &intruso, ix).is_err());

    // complete por intruso
    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_task_manager::instruction::CompleteTask {}.data(),
        solana_task_manager::accounts::CompleteTask {
            creator: intruso.pubkey(),
            task: task0,
        }
        .to_account_metas(None),
    );
    assert!(send_with_signer(&mut svm, &payer, &intruso, ix).is_err());

    // delete por intruso
    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_task_manager::instruction::DeleteTask {}.data(),
        solana_task_manager::accounts::DeleteTask {
            creator: intruso.pubkey(),
            task: task0,
        }
        .to_account_metas(None),
    );
    assert!(send_with_signer(&mut svm, &payer, &intruso, ix).is_err());

    // La tarea sigue intacta y sin completar
    let t = read_task(&svm, task0);
    assert_eq!(t.title, "Tarea propia");
    assert!(!t.completed);
}

#[test]
fn test_validacion_titulo_vacio_se_rechaza() {
    let (mut svm, payer, profile, program_id) = setup();
    init_profile(&mut svm, &payer, profile, program_id);
    let task0 = task_pda(&payer.pubkey(), 0, &program_id);
    let ix = Instruction::new_with_bytes(
        program_id,
        &solana_task_manager::instruction::CreateTask {
            title: "   ".to_string(),
            description: "desc".to_string(),
        }
        .data(),
        solana_task_manager::accounts::CreateTask {
            creator: payer.pubkey(),
            profile,
            task: task0,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    assert!(send(&mut svm, &payer, ix).is_err());
    assert!(svm.get_account(&task0).is_none());
}
