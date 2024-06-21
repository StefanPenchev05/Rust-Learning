"use server";

import Text from "./Text";
import List from "./List";

import { FaGithub, FaEnvelope, FaInstagram } from "react-icons/fa";

function NavBar() {
  const getEmailBody = (path: string): string => {
    // API call to the server for email tamplate

    return "[Your message]";
  };

  return (
    <nav className="flex flex-col md:flex-row md:justify-between justify-center items-center w-full md:p-4 p-6 border-b-4 border-b-black shadow-lg">
      <Text TextStyle={"head"} text="Actix with React" />
      <List
        position={"horizontal"}
        parentDivStyle={"justify-center md:space-x-24 my-2"}
      >
        <a
          href="https://instagram.com/_stefan.penchev_/"
          target="_blank"
          rel="noopener noreferrer"
          className="flex items-center"
        >
          <FaInstagram size={40} />
        </a>
        <a
          href="https://github.com/StefanPenchev05"
          target="_blank"
          rel="noopener noreferrer"
        >
          <FaGithub size={40} />
        </a>
        <a
          href={`mailto:penchev.stefan@icloud.com?subject=Email from Actix with React&body=${getEmailBody(
            "../EmailBody/SendEmail.txt"
          )}`}
        >
          <FaEnvelope size={40} />
        </a>
      </List>

      <Text
        TextStyle={"custom"}
        text="Design By Stefan Penchev"
        className={
          "animate-reveal overflow-hidden whitespace-nowrap max-w-max italic"
        }
      />
    </nav>
  );
}

export default NavBar;
