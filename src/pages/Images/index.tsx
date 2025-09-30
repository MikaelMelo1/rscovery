import { Link, useLocation, useParams } from "react-router-dom";
import "./styles.css";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface ImagePayload {
  iteration: number;
  base64: string;
}

export default function Images() {
  const { search } = useLocation();
  const queryParams = new URLSearchParams(search);

  const id = queryParams.get("id");

  const [loadingScan, setLoadingScan] = useState(false);
  const [images, setImages] = useState<string[]>([]);
  const [iteration, setIteration] = useState(0);

  useEffect(() => {
    const unlistenProgress = listen("file-found", (event) => {
      const progress = event.payload as ImagePayload;
      console.log(progress)
      setImages((prev) => [...prev, progress.base64]);
      setIteration(progress.iteration);
    });


    return () => {
      unlistenProgress.then((f) => f());
    };
  }, []);

  const handleStartScan = async () => {
    try {
      // console.log(id);
      setLoadingScan(true);
      await invoke("find_jpeg", { path: id });
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
        <h1>📷 Disk "{id}"</h1>
      </header>

      {loadingScan ? (
        <div>
          <p>
            {iteration}
        </p>
          <div className="scanGrid">
            {images.map((img, index) => (
                    <img    
                key={index}
                        src={`data:image/jpeg;base64,${img}`}
                        alt={`Recovered ${index}`}
                        className="recoveredImage"
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
