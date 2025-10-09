import { HashRouter, Routes, Route } from "react-router-dom";
import Main from "./pages/Main";

import "./App.css"
import Scanning from "./pages/Scanning";
import Images from "./pages/Images";
import Files from "./pages/Files";
import Text from "./pages/Text";
import Blocks from "./pages/Blocks";

function App() {
  return (
    <HashRouter>
      <Routes>
        <Route path="/" element={<Main />} />
        <Route path="/disk" element={<Scanning />} />
        <Route path="/images" element={<Images />} />
        <Route path="/file" element={<Files />} />
        <Route path="/text" element={<Text />} />
        <Route path="/blocks" element={<Blocks />} />
      </Routes>
    </HashRouter>
  );
}

export default App;
