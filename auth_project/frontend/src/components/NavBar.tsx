import Text from "./Text";
import List from "./List";

import { FaGithub, FaEnvelope } from "react-icons/fa";

function NavBar() {
  return (
    <nav className="flex justify-between items-center w-full p-4 border-b-4 border-b-black shadow-lg">
      <Text TextStyle={"head"} text="Actix with React" />
      <List position={"horizontal"}>
        <FaGithub size={20} />
        <FaEnvelope size={20} />
      </List>

      <Text
        TextStyle={"custom"}
        text="Design By Stefan Penchev"
        style={
          "animate-reveal overflow-hidden whitespace-nowrap max-w-max italic"
        }
      />
    </nav>
  );
}

export default NavBar;
