import { NextApiRequest, NextApiResponse } from "next";

export async function GET(res : NextApiResponse) {
    return res.json({
        msg:"Hello"
    })
}