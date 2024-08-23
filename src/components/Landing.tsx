import { Button } from "./ui/button"
import { Card, CardContent, CardDescription, CardFooter, CardHeader } from "./ui/card"

export const Landing = () => {
    return(
       <div className="flex w-full min-h-[90vh] mt-5 text-center justify-center relative items-center flex-col bg-[url('/grid.png')] bg-center bg-no-repeat">
        {/* <div className="absolute z-100 blur-[100px] bg-transparent top-0 left-0 right-0 transform-translate-x-[-50%] w-[80%] h-[15%] bg-violet-300"></div> */}
         <Card className="flex md:w-[50%] mt-[-15%] w-full border-none bg-transparent justify-center items-center flex-col p-4 gap-y-1">
            <CardHeader className="text-5xl font-bold">
                Lorem ipsum dolor sit amet.
            </CardHeader>
            <CardContent className="text-2xl text-white/80">
                Lorem ipsum dolor, sit amet consectetur adipisicing elit. Obcaecati, sit?
            </CardContent>
            <CardFooter>
                <Button variant={"default"}>
                    Get Started
                </Button>
            </CardFooter>
        </Card>
       </div>
    ) 
}