import { Features } from "@/components/features";
import { Landing } from "@/components/Landing";
import Image from "next/image";

export default function Home() {
  return (
   <div className="min-h-[90vh] flex max-w-screen justify-center items-center flex-col">
    <Landing/>
    <Features/>
   </div>
  );
}
