import Head from "next/head";
import axios from "axios";
import styles from "@/styles/Menu.module.css";
import { Inter } from "next/font/google";
import { useEffect, useState } from "react";
import { useRouter } from "next/router";
import { useAuth } from "../context/authContext";

const inter = Inter({ subsets: ["latin"] });

export default function Menu() {
  const router = useRouter();
  const { user, loading } = useAuth();
  const [menus, setMenus] = useState([]);
  const [tableId, setTableId] = useState(null);
  const [isSending, setIsSending] = useState(false);

  useEffect(() => {
    const { tableId } = router.query;
    if (!loading && !user) {
      router.push("/login");
    }
    if (!loading && user && tableId) {
      setTableId(tableId);
      axios
        .get(`/api/menu`, {
          headers: {
            Authorization: `Bearer ${user.token}`,
          },
        })
        .then((res) => {
          console.log(res.data);
          setMenus(res.data);
        })
        .catch((error) => {
          alert("Error fetching data: ", error);
        });
    }
  }, [user, loading, router]);

  const addOrder = (tableId, menuId, menuName) => {
    setIsSending(true);
    console.log("Menu ID:", menuId);
    axios
      .post(`/api/order`,
      {
          restaurant_table_id: parseInt(tableId),
          menu_id: parseInt(menuId),
      },
      {
        headers: {
          Authorization: `Bearer ${user.token}`,
          "Content-Type": "application/json",
        },
      })
      .then((res) => {
        console.log(res.data);
        alert(`"${menuName}" was added to the order`);
        setIsSending(false);
      })
      .catch((error) => {
        alert("Error fetching data: ", error);
        setIsSending(false);
      });
  };

  const toMinutesAndSeconds = (totalSeconds) => {
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;

    return `${minutes} minutes`;
  }

  return (
    <>
      <Head>
        <title>Restaurant App</title>
        <meta name="description" content="Restaurant App" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main className={`${styles.main} ${inter.className}`}>
        <h2>Table {tableId}</h2>
        <ul>
          {menus.length === 0 ? (
            <div>No Menu found</div>
          ) : (
            menus.map((menu, index) => (
              <li className={`${styles.menu}`} key={index}>
                <ul>
                  <li>{menu.name}</li>
                  <li>expected to take {toMinutesAndSeconds(menu.cook_time_seconds)}</li>
                  <li>{menu.price} yen</li>
                  <li>
                    <button
                      className={`${styles.deleteOrderButton}`}
                      type="button"
                      onClick={() => addOrder(tableId, menu.id, menu.name)}
                      disabled={isSending}
                    >
                      Add this to order
                    </button>
                  </li>
                </ul>
              </li>
            ))
          )}
        </ul>
      </main>
    </>
  );
}
