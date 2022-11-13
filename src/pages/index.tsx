import axios, { AxiosResponse } from 'axios'
import type { NextPage } from 'next'
import Head from 'next/head'
import { useEffect, useState } from 'react'

const Home: NextPage = () => {
  const [data, setData] = useState<AxiosResponse>()
  useEffect(() => {
    axios.get('/api/movie')
      .then(data => setData(data))
      .catch(err => console.log(err))
  })
  return (
    <div>
      <Head>
        <title>Create Next App</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <div>
        {data?.data}
      </div>
    </div>
  )
}

export default Home
