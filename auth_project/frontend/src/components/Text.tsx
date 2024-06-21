import React from "react";

type Text =
  | {
      TextStyle: "head" | "para";
      text: string;
      parentDivStyle?: string;
      style?: string;
    }
  | {
      TextStyle: "custom" | undefined;
      text: string;
      parentDivStyle?: string;
      style: string;
    };

const Text: React.FC<Text> = ({ ...args }) => {
  const style: string = [
    {
      head: "text-2xl font-bold",
      para: "text-base",
      custom: args.style,
    },
  ][0][args.TextStyle == undefined ? "para" : args.TextStyle] as string;

  return (
    <div className={args.parentDivStyle}>
      <p className={style}>{args.text}</p>
    </div>
  );
};

export default Text;
