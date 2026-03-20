import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/core";

type ApiResponse<T> = {
  ok: boolean;
  message: string;
  data: T;
};

type HostEntry = {
  ip: string;
  hostname: string;
  comment?: string | null;
};

export default function App() {
  const [hostIp, setHostIp] = createSignal("127.0.0.1");
  const [hostName, setHostName] = createSignal("localhost.localdomain");
  const [hostComment, setHostComment] = createSignal("stub entry");
  const [port, setPort] = createSignal("3000");
  const [output, setOutput] = createSignal("Command output will appear here.");
  const [busy, setBusy] = createSignal(false);

  const callCommand = async <T,>(command: string, args?: Record<string, unknown>) => {
    setBusy(true);
    try {
      const result = await invoke<ApiResponse<T>>(command, args);
      setOutput(JSON.stringify(result, null, 2));
    } catch (error) {
      setOutput(`Invoke error: ${String(error)}`);
    } finally {
      setBusy(false);
    }
  };

  return (
    <main class="layout">
      <section class="panel">
        <h1>DevHosts</h1>
        <p>Foundation build with placeholder Tauri command wiring.</p>
      </section>

      <section class="panel">
        <h2>Hosts Commands</h2>
        <div class="grid">
          <label>
            IP
            <input value={hostIp()} onInput={(event) => setHostIp(event.currentTarget.value)} />
          </label>
          <label>
            Hostname
            <input value={hostName()} onInput={(event) => setHostName(event.currentTarget.value)} />
          </label>
          <label>
            Comment
            <input value={hostComment()} onInput={(event) => setHostComment(event.currentTarget.value)} />
          </label>
        </div>
        <div class="actions">
          <button disabled={busy()} onClick={() => callCommand<HostEntry[]>("list_hosts")}>
            list_hosts
          </button>
          <button
            disabled={busy()}
            onClick={() =>
              callCommand("add_host", {
                request: { ip: hostIp(), hostname: hostName(), comment: hostComment() }
              })
            }
          >
            add_host
          </button>
          <button
            disabled={busy()}
            onClick={() =>
              callCommand("remove_host", {
                hostname: hostName()
              })
            }
          >
            remove_host
          </button>
          <button disabled={busy()} onClick={() => callCommand("clean_hosts")}>
            clean_hosts
          </button>
        </div>
      </section>

      <section class="panel">
        <h2>Port Commands</h2>
        <div class="grid">
          <label>
            Port
            <input value={port()} onInput={(event) => setPort(event.currentTarget.value)} />
          </label>
        </div>
        <div class="actions">
          <button disabled={busy()} onClick={() => callCommand("list_ports")}>
            list_ports
          </button>
          <button
            disabled={busy()}
            onClick={() =>
              callCommand("kill_port", {
                port: Number.parseInt(port(), 10) || 0
              })
            }
          >
            kill_port
          </button>
        </div>
      </section>

      <section class="panel output">
        <h2>Output</h2>
        <pre>{output()}</pre>
      </section>
    </main>
  );
}

