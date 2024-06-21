import React from "react";

type TextProps =
  | {
      TextStyle: "head" | "para";
      text: string;
      parentDivStyle?: string;
      className?: string;
    }
  | {
      TextStyle: "custom" | undefined;
      text: string;
      parentDivStyle?: string;
      className: string;
    };

const Text: React.FC<TextProps> = ({ ...args }) => {
  const style: string = [
    {
      head: "text-lg md:text-2xl font-bold",
      para: "text-sm md:text-base",
      custom: args.className,
    },
  ][0][args.TextStyle === undefined ? "para" : args.TextStyle] as string;

  return (
    <div className={args.parentDivStyle}>
      <p className={style}>{args.text}</p>
    </div>
  );
};

export default Text;