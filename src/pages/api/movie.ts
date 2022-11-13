import axios from "axios";
import { NextApiRequest, NextApiResponse } from "next";

export default async function handler(req: NextApiRequest, res: NextApiResponse) {
  const data = await axios.get("http://localhost:8000/api/movie")
  res.status(200).json(data.data)
}