import { useCallback, useEffect, useMemo, useState } from "react";
import { useAnchorWallet, useConnection, useWallet } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { AnchorProvider, Program } from "@anchor-lang/core";
import { PublicKey } from "@solana/web3.js";
import idl from "./idl.json";
import type { SolanaTaskManager } from "./idlType";
import { completeTask, createTask, deleteTask, initializeProfile, listMyTasks, profilePda, updateTask } from "./client";

type TaskRow = { publicKey: PublicKey; account: { id: bigint; title: string; description: string; completed: boolean; creator: PublicKey; createdAt: bigint } };

export default function App() {
  const { connection } = useConnection();
  const wallet = useAnchorWallet();
  const { publicKey } = useWallet();
  const [tasks, setTasks] = useState<TaskRow[]>([]);
  const [hasProfile, setHasProfile] = useState(false);
  const [title, setTitle] = useState("");
  const [desc, setDesc] = useState("");
  const [msg, setMsg] = useState("");
  const [editing, setEditing] = useState<PublicKey | null>(null);

  const program = useMemo(() => {
    if (!wallet) return null;
    const provider = new AnchorProvider(connection, wallet, { commitment: "confirmed" });
    return new Program(idl as SolanaTaskManager, provider);
  }, [connection, wallet]);

  const refresh = useCallback(async () => {
    if (!program || !publicKey) return;
    try {
      const [profile] = profilePda(publicKey);
      await program.account.profile.fetch(profile);
      setHasProfile(true);
      setTasks(await listMyTasks(program) as TaskRow[]);
    } catch { setHasProfile(false); setTasks([]); }
  }, [program, publicKey]);

  useEffect(() => { refresh(); }, [refresh]);

  const run = async (fn: () => Promise<unknown>, ok: string) => {
    try { setMsg("Firmando…"); const sig = await fn(); setMsg(`${ok} · ${String(sig)}`); await refresh(); }
    catch (e: unknown) { setMsg(`Error: ${e instanceof Error ? e.message : String(e)}`); }
  };

  return (
    <div style={{ maxWidth: 720, margin: "0 auto", padding: 24, fontFamily: "system-ui" }}>
      <h1>Gestor de Tareas · Solana Devnet</h1>
      <p>Programa: <code>3T8VSbh6k1sEStgASQ3epLhCqdP2dv4WxVyAD4RMPCqj</code> · Cada tarea es una PDA <code>["task", creador, id]</code> y solo se puede cerrar cuando está completada.</p>
      <WalletMultiButton />
      {!publicKey && <p>Conecta tu wallet (Phantom / Solflare) en Devnet para empezar.</p>}
      {publicKey && !hasProfile && (
        <p><button onClick={() => program && run(() => initializeProfile(program), "Perfil creado")}>Crear mi perfil</button></p>
      )}
      {hasProfile && (
        <>
          <h2>Nueva tarea</h2>
          <input value={title} onChange={(e) => setTitle(e.target.value)} placeholder="Título (máx. 64)" style={{ width: "100%", padding: 8 }} />
          <input value={desc} onChange={(e) => setDesc(e.target.value)} placeholder="Descripción (máx. 200)" style={{ width: "100%", padding: 8, marginTop: 8 }} />
          <p><button onClick={() => program && run(async () => { const r = await createTask(program, title, desc); setTitle(""); setDesc(""); return r.sig; }, "Tarea creada")}>Crear tarea</button></p>
          <h2>Mis tareas ({tasks.length})</h2>
          {tasks.map((t) => (
            <div key={t.publicKey.toBase58()} style={{ border: "1px solid #ccc", borderRadius: 8, padding: 12, marginBottom: 8 }}>
              <b>{t.account.completed ? "✅ " : "⬜ "}{t.account.title}</b> <small>#{String(t.account.id)}</small>
              <p>{t.account.description}</p>
              <small>{t.publicKey.toBase58()}</small>
              <div style={{ display: "flex", gap: 8, marginTop: 8 }}>
                {!t.account.completed && <button onClick={() => program && run(() => completeTask(program, t.publicKey), "Tarea completada")}>Completar</button>}
                {!t.account.completed && <button onClick={() => { setEditing(t.publicKey); setTitle(t.account.title); setDesc(t.account.description); }}>Editar</button>}
                {editing?.equals(t.publicKey) && <button onClick={() => program && run(() => updateTask(program, t.publicKey, title, desc), "Tarea actualizada")}>Guardar edición</button>}
                <button onClick={() => program && run(() => deleteTask(program, t.publicKey), "Tarea eliminada (renta devuelta)")}>Eliminar</button>
              </div>
              {!t.account.completed && <p><i>Si intentas eliminar sin completar, el programa lo rechaza con <code>TaskNotCompleted</code>.</i></p>}
            </div>
          ))}
        </>
      )}
      {msg && <p><b>{msg}</b></p>}
      <p><a href="https://explorer.solana.com/address/3T8VSbh6k1sEStgASQ3epLhCqdP2dv4WxVyAD4RMPCqj?cluster=devnet" target="_blank" rel="noreferrer">Ver programa en Solana Explorer (Devnet)</a></p>
    </div>
  );
}
