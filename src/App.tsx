import { HashRouter, Routes, Route } from "react-router-dom";
import Main from "./pages/Main";

import "./App.css"
import Scanning from "./pages/Scanning";

function App() {
  return (
    <HashRouter>
      <Routes>
        <Route path="/" element={<Main />} />
        <Route path="/disk/:id" element={<Scanning />} />
      </Routes>
    </HashRouter>
  );
}

export default App;
