# Gestor de Tareas on-chain — Solana (Devnet)

**Ruta elegida: proyecto propio.** No es el registro de ventas guiado: es un gestor de tareas con otra temática, construido desde el template de Anchor 1.2.2, con programa, cliente e frontend propios.

**Autor / certificado:** Diego — correo de registro: deilodas418@gmail.com

## Propósito
Cada usuario crea su perfil y sus tareas como cuentas on-chain. Una tarea solo la puede modificar su creador y **solo se puede eliminar (cerrar la cuenta, con devolución de la renta) cuando ya está completada**. Esa es la regla de negocio que se prueba y se demuestra.

## Stack
Anchor 1.2.2 · Solana CLI 4.4.1 · Rust 1.89 (pin en `rust-toolchain.toml`) · `@anchor-lang/core` 1.2.2 · `@solana/web3.js` v1 · React + Vite + wallet-adapter.

## Programa
- **Program ID (Devnet, el mismo que en `Anchor.toml` y en el cliente):** `3T8VSbh6k1sEStgASQ3epLhCqdP2dv4WxVyAD4RMPCqj`
- Explorer: https://explorer.solana.com/address/3T8VSbh6k1sEStgASQ3epLhCqdP2dv4WxVyAD4RMPCqj?cluster=devnet
- Cuentas (PDAs, relacionadas):
  - `Profile` — PDA `["profile", owner]` — `owner`, `task_count` (id de la próxima tarea).
  - `Task` — PDA `["task", creator, id.to_le_bytes()]` — `creator`, `id`, `title` (≤64), `description` (≤200), `completed`, `created_at`, `bump`.
- Instrucciones (`programs/solana-task-manager/src/instructions/`):
  1. `initialize_profile` — crea el perfil, una vez por wallet.
  2. `create_task(title, description)` — crea la tarea con `id = task_count` y suma el contador. Rechaza título vacío o demasiado largo.
  3. `update_task(title, description)` — solo creador (`has_one = creator`), solo si no está completada.
  4. `complete_task` — solo creador, solo si no estaba completada.
  5. `delete_task` — solo creador y **solo si `completed == true`** (`TaskNotCompleted` en caso contrario). Usa `close = creator`: cierra la cuenta y devuelve la renta al creador.

## Aporte propio
El template de Anchor solo traía un contador (`initialize`/`increment`). Todo lo demás es propio: el modelo Profile/Task con PDAs relacionadas, las 5 instrucciones, las validaciones de negocio en Rust, el cierre condicionado, las pruebas de éxito y de rechazo, el cliente TS y el frontend.

## Configuración y ejecución
```bash
# Requisitos: Rust (rustup), Solana CLI, Anchor 1.2.2, Node 20+
solana config set --url devnet
solana-keygen new -o ~/.config/solana/id.json   # si no tienes wallet
solana airdrop 2

anchor build
anchor test          # pruebas Rust con LiteSVM (ver abajo)
anchor deploy --provider.cluster devnet
# Copiar el IDL generado:
cp target/idl/solana_task_manager.json client/idl.json
cp target/idl/solana_task_manager.json app/src/idl.json
cp target/types/solana_task_manager.ts client/src/idlType.ts
cp target/types/solana_task_manager.ts app/src/idlType.ts

# Frontend (Devnet, wallet Phantom/Solflare en Devnet)
cd app && npm install && npm run dev   # http://localhost:5173
# Cliente CLI (opcional)
cd client && npm install && npm run demo
```

## Cómo probarlo (flujo)
1. Conecta la wallet en Devnet → **Crear mi perfil**.
2. Crea una tarea → aparece en la lista (lectura con `program.account.task.all`, filtrada por creador).
3. Edítala → se actualiza en cadena.
4. Intenta **Eliminar sin completar** → la transacción falla con `TaskNotCompleted` (el frontend lo muestra).
5. **Completar** → **Eliminar** → la cuenta se cierra y la renta vuelve a tu wallet. Verifícalo en el Explorer con la firma de la transacción.

## Pruebas y evidencias
- `programs/solana-task-manager/tests/test_tasks.rs` (LiteSVM, `anchor test` / `cargo test`):
  - `test_flujo_completo_crear_leer_actualizar_completar_y_cerrar` — flujo feliz + rechazo de borrado antes de completar + rechazo de editar una tarea completada + cierre verificado (`get_account == None`).
  - `test_permisos_otro_usuario_no_puede_modificar` — un intruso no puede actualizar, completar ni borrar.
  - `test_validacion_titulo_vacio_se_rechaza` — título vacío no crea la cuenta.
- **Estado de la compilación al entregar este repo:** `anchor build` estaba compilando las dependencias de Anchor/SBF en esta máquina (1,5 CPU, primera compilación); el código del programa, las pruebas, el cliente y el frontend están completos en el repo. Tras `anchor build`, el IDL real sustituye al placeholder de `client/idl.json` y `app/src/idl.json`, y el despliegue en Devnet se hace con `anchor deploy --provider.cluster devnet` (el Program ID ya está reservado en `Anchor.toml`, en `lib.rs` y en el cliente). Las capturas / firmas de Devnet se añaden en `evidence/` tras el despliegue.
- `evidence/` — carpeta para las capturas y las firmas verificables de Devnet.

## Identificación
- **Nombre para el certificado:** Diego
- **Correo utilizado en el registro:** deilodas418@gmail.com
