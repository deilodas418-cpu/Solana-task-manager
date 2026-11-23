import { AnchorProvider, Program } from "@anchor-lang/core";
import { Connection, PublicKey } from "@solana/web3.js";
import idl from "../idl.json" with { type: "json" };
import type { SolanaTaskManager } from "./idlType";

export const PROGRAM_ID = new PublicKey("3T8VSbh6k1sEStgASQ3epLhCqdP2dv4WxVyAD4RMPCqj");
export const DEVNET = "https://api.devnet.solana.com";

export function profilePda(owner: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync([Buffer.from("profile"), owner.toBuffer()], PROGRAM_ID);
}
export function taskPda(creator: PublicKey, id: number | bigint): [PublicKey, number] {
  const buf = Buffer.alloc(8);
  buf.writeBigUInt64LE(BigInt(id));
  return PublicKey.findProgramAddressSync(
    [Buffer.from("task"), creator.toBuffer(), buf],
    PROGRAM_ID,
  );
}
export function getProgram(provider: AnchorProvider) {
  return new Program(idl as SolanaTaskManager, provider);
}
export function getReadonlyProgram(connection = new Connection(DEVNET, "confirmed")) {
  return new Program(idl as SolanaTaskManager, { connection });
}

/** Flujo completo, usado por el frontend y por el script demo. */
export async function initializeProfile(program: Program<SolanaTaskManager>) {
  const owner = program.provider.publicKey!;
  const [profile] = profilePda(owner);
  return program.methods.initializeProfile().accounts({ owner, profile }).rpc();
}
export async function createTask(program: Program<SolanaTaskManager>, title: string, description: string) {
  const creator = program.provider.publicKey!;
  const [profile] = profilePda(creator);
  const prof = await program.account.profile.fetch(profile);
  const [task] = taskPda(creator, prof.taskCount);
  const sig = await program.methods.createTask(title, description)
    .accounts({ creator, profile, task }).rpc();
  return { sig, task, id: Number(prof.taskCount) };
}
export async function updateTask(program: Program<SolanaTaskManager>, task: PublicKey, title: string, description: string) {
  const creator = program.provider.publicKey!;
  return program.methods.updateTask(title, description).accounts({ creator, task }).rpc();
}
export async function completeTask(program: Program<SolanaTaskManager>, task: PublicKey) {
  const creator = program.provider.publicKey!;
  return program.methods.completeTask().accounts({ creator, task }).rpc();
}
export async function deleteTask(program: Program<SolanaTaskManager>, task: PublicKey) {
  const creator = program.provider.publicKey!;
  return program.methods.deleteTask().accounts({ creator, task }).rpc();
}
export async function listMyTasks(program: Program<SolanaTaskManager>) {
  const creator = program.provider.publicKey!;
  const all = await program.account.task.all([
    { memcmp: { offset: 8, bytes: creator.toBase58() } },
  ]);
  return all.sort((a, b) => Number(a.account.id - b.account.id));
}
