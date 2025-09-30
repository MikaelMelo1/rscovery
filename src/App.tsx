import { HashRouter, Routes, Route } from "react-router-dom";
import Main from "./pages/Main";

import "./App.css"
import Scanning from "./pages/Scanning";
import Images from "./pages/Images";

function App() {
  return (
    <HashRouter>
      <Routes>
        <Route path="/" element={<Main />} />
        <Route path="/disk" element={<Scanning />} />
        <Route path="/images" element={<Images />} />
      </Routes>
    </HashRouter>
  );
}

export default App;
