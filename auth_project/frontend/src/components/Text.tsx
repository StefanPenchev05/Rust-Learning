import React, { ReactNode } from "react";

type TextProps = {
  TextStyle?: "head" | "head-2" | "head-3" | "para" | "custom";
  children: ReactNode;
  parentDivStyle?: string;
  className?: string;
};
const Text: React.FC<TextProps> = ({
  TextStyle,
  children,
  parentDivStyle,
  className,
}) => {
  const style: string = [
    {
      head: "text-lg md:text-2xl font-bold",
      "head-2": "text-xl md:text-2xl font-extrabold",
      "head-3": "text-lg md:text-3xl font-extrabold",
      para: "text-sm md:text-base",
      custom: className,
    },
  ][0][TextStyle === undefined ? "para" : TextStyle] as string;

  return (
    <div className={parentDivStyle}>
      <p className={style}>{children}</p>
    </div>
  );
};

export default Text;
