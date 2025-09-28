import { Link, useParams } from "react-router-dom";
import "./styles.css";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface ScannedData {
  current: number;
  total: number;
  nonEmpty: number[];
}

export default function Scanning() {
  const { id } = useParams<{ id: string }>();

  const [loadingScan, setLoadingScan] = useState(false);
  const [scannedSize, setScannedSize] = useState<ScannedData>({
    current: 0,
    total: 0,
    nonEmpty: [],
  });

  useEffect(() => {
    const unlistenProgress = listen("scan-progress", (event) => {
      console.log("Progresso:", event.payload);
    });

    // cleanup
    return () => {
      unlistenProgress.then((f) => f());
    };
  }, []);

  const handleStartScan = async () => {
    try {
      // console.log(id);
      await invoke("analyze_blocks", { path: id });
    } catch (err) {
      console.error("Error starting scan:", err);
    }
    setLoadingScan(true);
  };

  return (
    <main className="container">
      <header>
        <div>
          <Link to={"/"}>Go Back</Link>
        </div>
        <h1>Disk "{id}"</h1>
      </header>

      {loadingScan ? (
        <div>
          <p>
            {(scannedSize.current / 1024).toFixed(2)}/
            {(scannedSize.total / 1024).toFixed(2)} GB
          </p>
          <div className="scanGrid">
            {Array.from({ length: 200 }, (_, i) => (
              <div
                style={
                  scannedSize.nonEmpty.includes(i)
                    ? { backgroundColor: "rgb(142, 255, 168)" }
                    : scannedSize.current / scannedSize.total > i / 200
                    ? { backgroundColor: "rgb(112, 86, 86)" }
                    : {}
                }
                key={i}
              />
            ))}
          </div>
        </div>
      ) : (
        <div style={{ marginTop: "24px", textAlign: "center" }}>
          <button onClick={handleStartScan}>Start Scan</button>
        </div>
      )}
    </main>
  );
}
