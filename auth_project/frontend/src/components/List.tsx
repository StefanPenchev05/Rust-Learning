import React, { ReactNode } from "react";

interface List {
  position: "horizontal" | "vertical";
  children: ReactNode;
}

const List: React.FC<List> = ({ position, children }) => {
  const style:string = [
    {
      horizontal: "flex flex-row justify-between items-center",
      vertical: "flex flex-col justify-between items-center",
    },
  ][0][position];

  return (
    <div className={style}>
      {children}
    </div>
  );
};

export default List;
