import Head from "next/head";
import axios from "axios";
import styles from "@/styles/Order.module.css";
import { Inter } from "next/font/google";
import { useEffect, useState } from "react";
import { useRouter } from "next/router";
import { useAuth } from "../context/authContext";
import Countdown from "../components/countdown";

const inter = Inter({ subsets: ["latin"] });

export default function Order() {
  const router = useRouter();
  const { user, loading } = useAuth();
  const [orders, setOrders] = useState([]);
  const [tableId, setTableId] = useState(null);

  useEffect(() => {
    const { tableId } = router.query;
    if (!loading && !user) {
      router.push("/login");
    }
    if (!loading && user && tableId) {
      setTableId(tableId);
      axios
        .get(`http://localhost:8080/api/table/${tableId}/order`, {
          headers: {
            Authorization: `Bearer ${user.token}`,
          },
        })
        .then((res) => {
          console.log(res.data);
          setOrders(res.data);
        })
        .catch((error) => {
          alert("Error fetching data: ", error);
        });
    }
  }, [user, loading, router]);

  const deleteOrder = (orderId) => {
    console.log("Order ID:", orderId);
    axios
      .delete(`http://localhost:8080/api/order`, {
        headers: {
          Authorization: `Bearer ${user.token}`,
          "Content-Type": "application/json",
        },
        data: {
          order_id: parseInt(orderId),
        },
      })
      .then((res) => {
        console.log(res.data);
        setOrders((prevOrders) =>
          prevOrders.filter((order) => order.order_id !== orderId)
        );
      })
      .catch((error) => {
        alert("Error fetching data: ", error);
      });
  };

  const serveOrder = (orderId, menuName) => {
    console.log("Order ID:", orderId);
    axios
      .delete(`http://localhost:8080/api/order/complete`, {
        headers: {
          Authorization: `Bearer ${user.token}`,
          "Content-Type": "application/json",
        },
        data: {
          order_id: parseInt(orderId),
          user_name: user.sub,
        },
      })
      .then((res) => {
        console.log(res.data);
        setOrders((prevOrders) =>
          prevOrders.filter((order) => order.order_id !== orderId)
        );
        alert(`Successfully served "${menuName}"`);
      })
      .catch((error) => {
        alert("Error fetching data: ", error);
      });
  };

  const deleteAllOrders = (tableId) => {
    console.log("Table ID:", tableId);
    axios
      .delete(`http://localhost:8080/api/table/order`, {
        headers: {
          Authorization: `Bearer ${user.token}`,
          "Content-Type": "application/json",
        },
        data: {
          restaurant_table_id: parseInt(tableId),
        },
      })
      .then((res) => {
        console.log(res.data);
        setOrders([]);
        alert(`Successfully canceled all orders`);
      })
      .catch((error) => {
        alert("Error fetching data: ", error);
      });
  };

  const goToMenuPage = (tableId) => {
    console.log("Table ID:", tableId);
    router.push(`/menu?tableId=${tableId}`);
  };

  const toLocalTime = (utcTimeString) => {
    // Create a Date object from the UTC time string
    const date = new Date(ensureUtc(utcTimeString));

    // Format the date to a string representing local time
    const localTimeString = date.toLocaleString();
    return localTimeString;
  };

  const ensureUtc = (timeString) => {
    if (!timeString.endsWith("Z")) {
      return timeString + "Z";
    }
    return timeString;
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
        <h2>Table {tableId}</h2>
        <ul>
          {orders.length === 0 ? (
            <div>No order found</div>
          ) : (
            orders.map((order, index) => (
              <li className={`${styles.order}`} key={index}>
                <ul>
                  <li>{order.menu_name}</li>
                  <li>
                    Expected to finish cooking at:{" "}
                    {toLocalTime(order.expected_cook_finish_time)}
                  </li>
                  <li>
                    <Countdown
                      targetTimeString={toLocalTime(
                        order.expected_cook_finish_time
                      )}
                    />
                  </li>
                  <li>
                    <button
                      className={`${styles.deleteOrderButton}`}
                      type="button"
                      onClick={() => deleteOrder(order.order_id)}
                    >
                      cancel order
                    </button>
                    <button
                      className={`${styles.deleteOrderButton}`}
                      type="button"
                      onClick={() =>
                        serveOrder(order.order_id, order.menu_name)
                      }
                    >
                      serve
                    </button>
                  </li>
                </ul>
              </li>
            ))
          )}
        </ul>
        <div>
          <button type="button" onClick={() => goToMenuPage(tableId)}>
            Menu
          </button>
          <button
            type="button"
            className={`${styles.deleteOrderButton}`}
            onClick={() => deleteAllOrders(tableId)}
          >
            Delete all orders
          </button>
        </div>
      </main>
    </>
  );
}
