import Image from "next/image";
import { hello } from "./api/action/route";

export default function Home() {

  hello()
  return (
   <>
    
   </>
  );
}
