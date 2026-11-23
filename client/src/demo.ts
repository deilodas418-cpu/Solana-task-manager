// Demo por CLI: requiere idl.json generado por `anchor build` y una wallet en ~/.config/solana/id.json
// Uso: npm run demo  (tras copiar target/idl/solana_task_manager.json a client/idl.json)
import { Connection, Keypair } from "@solana/web3.js";
import { AnchorProvider, Wallet } from "@anchor-lang/core";
import fs from "node:fs";
import os from "node:os";
import { createTask, completeTask, deleteTask, getProgram, initializeProfile, listMyTasks } from "./client.js";
const secret = JSON.parse(fs.readFileSync(`${os.homedir()}/.config/solana/id.json`, "utf8"));
const wallet = new Wallet(Keypair.fromSecretKey(Uint8Array.from(secret)));
const provider = new AnchorProvider(new Connection("https://api.devnet.solana.com", "confirmed"), wallet, {});
const program = getProgram(provider);
await initializeProfile(program).catch(() => console.log("perfil ya existía"));
const { task, id } = await createTask(program, "Demo CLI", "Creada desde client/src/demo.ts");
console.log("tarea", id, task.toBase58());
await completeTask(program, task);
await deleteTask(program, task);
console.log("flujo OK, tareas restantes:", (await listMyTasks(program)).length);
