import React, { ReactNode } from "react";

interface ListProps {
  position: "horizontal" | "vertical";
  children: ReactNode;
  parentDivStyle?: string
}

const List: React.FC<ListProps> = ({ position, children, parentDivStyle }) => {
  const itemsPosition:string = [
    {
      horizontal: "flex-row",
      vertical: "flex-col",
    },
  ][0][position] as string;

  return (
    <div className={`flex ${itemsPosition} items-center ${parentDivStyle}`}>
      {children}
    </div>
  );
};

export default List;
