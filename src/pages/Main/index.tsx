import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Link } from "react-router-dom";

type DiskInfo = {
  name: string;
  size: number;
};

export default function Main() {
  const [disks, setDisks] = useState<DiskInfo[]>([]);

  useEffect(() => {
    const fetchDisks = async () => {
      try {
        setDisks(await invoke<DiskInfo[]>("list_disks"));
      } catch (err) {
        console.log(err);
      }
    };

    fetchDisks();
  }, []);

  return (
    <main className="container">
      <h1>Rscovery 🦀</h1>
      <p>Select the device you want to recover.</p>

      {disks.length == 0 && (
        <div className="warningContainer">No devices found...</div>
      )}

      <div className="deviceList">
        {disks.map(({ name, size }) => (
          <Link
            to={`/disk?id=${name}`}
            key={name}
            style={{ textDecoration: "none", color: "inherit" }}
          >
            <div className="device">
              <h1>{name}</h1>
              <p>{(size / 1024).toFixed(2)} GB</p>
            </div>
          </Link>
        ))}
      </div>
    </main>
  );
}
