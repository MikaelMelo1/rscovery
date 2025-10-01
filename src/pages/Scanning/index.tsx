import { Link, useLocation } from "react-router-dom";
import "./styles.css";


const scanOptions = [{
  name: "JPEG",
  route: "/images?type=jpeg&id="
}, {
  name: "PNG",
  route: "/images?type=png&id="
}, {
  name: "PDF",
  route: "/file?type=pdf&id="
}, {
  name: "ZIP",
  route: "/file?type=zip&id="
}, {
  name: "Text",
  route: "/text?id="
},{
  name: "MP4",
  route: "/mp4?id="
},]

export default function Scanning() {
  const { search } = useLocation();
  const queryParams = new URLSearchParams(search);

  const id = queryParams.get("id");

  return (
    <main className="container">
      <header>
        <div>
          <Link to={"/"}>Go Back</Link>
        </div>
        <h1>Disk "{id}"</h1>
      </header>

    <div className="options">
      {scanOptions.map(({name, route}) => (
        <div key={name}>
      <Link to={`${route}${id}`}>
        <div>{name}</div>
      </Link>
      </div>
      ))}
    </div>

    </main>
  );
}
