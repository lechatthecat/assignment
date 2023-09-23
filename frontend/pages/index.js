import Head from 'next/head'
import styles from '@/styles/Home.module.css'
import { useAuth } from "../context/authContext";

export default function Home() {
  const { user, login, logout } = useAuth();
  return (
    <>
      <Head>
        <title>Restaurant App</title>
        <meta name="description" content="Restaurant App" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main className={`${styles.main} ${inter.className}`}>
        ssss
      </main>
    </>
  )
}
