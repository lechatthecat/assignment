import Head from 'next/head'
import axios from 'axios';
import styles from '@/styles/Home.module.css'
import { Inter } from 'next/font/google'
import { useEffect, useState } from 'react';
import { useRouter } from 'next/router';
import { useAuth } from "../context/authContext";

const inter = Inter({ subsets: ['latin'] })

export default function Home() {
  const router = useRouter();
  const { user, loading } = useAuth();
  const [tables, setTables] = useState([]);

  useEffect(() => {
    if (!loading && !user) {
      router.push('/login');
    }
    if (!loading && user) {
      axios.get('/api/table', {
        headers: {
          'Authorization': `Bearer ${user.token}`,
        },
      }).then(res => {
        console.log(res.data);
        setTables(res.data);
      }).catch(error => {
        alert("Error fetching data: ", error);
      })
    }
  }, [user, loading, router]);

  const goToOrderPage = (tableId) => {
    console.log("Table ID:", tableId);
    router.push(`/order?tableId=${tableId}`);
  };

  return (
    <>
      <Head>
        <title>Restaurant App</title>
        <meta name="description" content="Restaurant App" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main className={`${styles.main} ${inter.className}`}>
        <ul>
          {tables.map((table, index) => (
            <li
              className={`${styles.table}`}
              key={index}>
              Table {table.table_number}
              <button
                className={`${styles.showButton}`}
                type="button"
                onClick={() => goToOrderPage(table.id)}
              >
                  orders
              </button>
            </li>
          ))}
        </ul>
      </main>
    </>
  )
}
