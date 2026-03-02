import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import type { TimerState } from "./types/TimerState";

function App() {
  // O estado começa vazio (null) e depois recebe a struct do Rust
  const [timer, setTimer] = useState<TimerState | null>(null);

  // Assim que o app abrir, ele busca o estado inicial do Rust
  useEffect(() => {
    async function fetchTimerState() {
      // Pedimos pro Rust o estado e avisamos o TS que a resposta é do tipo TimerState
      const state = await invoke<TimerState>("get_timer_state");
      setTimer(state);
    }
    fetchTimerState();
  }, []);

  // Função auxiliar para transformar os segundos (ex: 1500) em "25:00"
  const formatTime = (totalSeconds: number) => {
    const minutes = Math.floor(totalSeconds / 60).toString().padStart(2, "0");
    const seconds = (totalSeconds % 60).toString().padStart(2, "0");
    return `${minutes}:${seconds}`;
  };

  return (
    <main className="container">
      <h1>Tomatche 🍅</h1>
      
      {/* Se o timer já carregou do Rust, mostramos na tela */}
      {timer ? (
        <div style={{ marginTop: "1rem" }}>
          <h2>{timer.current_phase === "Work" ? "Foco total!" : "Hora da pausa"}</h2>
          
          <div style={{ fontSize: "6rem", fontWeight: "bold", color: "#ff5252", textShadow: "2px 2px 4px rgba(0,0,0,0.5)" }}>
            {formatTime(timer.time_remaining)}
          </div>
          
          <p style={{ fontSize: "1.2rem", fontWeight: "500" }}>Ciclos completos: {timer.cycles}</p>
        </div>
      ) : (
        <p>Carregando o temporizador...</p>
      )}
    </main>
  );
}

export default App;