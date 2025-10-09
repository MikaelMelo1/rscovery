import { Link, useLocation } from "react-router-dom";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface ScannedData {
  current: number;
  total: number;
  nonEmpty: number[];
}

export default function Blocks() {
  const { search } = useLocation();
  const queryParams = new URLSearchParams(search);

  const id = queryParams.get("id");

  const [loadingScan, setLoadingScan] = useState(false);
  const [scannedSize, setScannedSize] = useState<ScannedData>({
    current: 0,
    total: 0,
    nonEmpty: [],
  });

  // nonEmpty shows the blocks index (the size is 32MB) that are non empty
  // we should convert it to the actual index.
  const viewBlockSize = scannedSize.total / 200;
  const nonEmptyBlocks = scannedSize.nonEmpty.map((index) => (index + 1) * 32);
  console.log({nonEmptyBlocks});


  useEffect(() => {
    const unlistenProgress = listen("scan-progress", (event) => {
      const progress = event.payload as ScannedData;
      console.log(progress)
      setScannedSize({
        current: progress.current,
        total: progress.total,
        nonEmpty: progress.nonEmpty,
      })
    });


    return () => {
      unlistenProgress.then((f) => f());
    };
  }, []);

  const handleStartScan = async () => {
    try {
      // console.log(id);
      setLoadingScan(true);
      await invoke("analyze_blocks", { path: id });
    } catch (err) {
      console.error("Error starting scan:", err);
    }
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
            {Array.from({ length: 200 }, (_, it) => {
              const i = it + 2;
              const cellRange = [(Math.max(0, (i - 1 ))) * viewBlockSize, (i + 1) * viewBlockSize];
              const nonEmptyCell = nonEmptyBlocks.find((value) => value >= cellRange[0] && value <= cellRange[1]);
              
              if(!nonEmptyCell) {
                console.log({cellRange, nonEmptyBlocks});
            }

              return (<div
                style={
                  nonEmptyCell
                    ? { backgroundColor: "rgb(142, 255, 168)" }
                    : scannedSize.current / scannedSize.total > i / 200
                    ? { backgroundColor: "rgb(116, 114, 114)" }
                    : {}
                }
                key={i}
              />
            )})}
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
