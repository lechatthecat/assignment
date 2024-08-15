import { useState, useEffect } from "react";
import axios from "axios";

const toMinutesAndSeconds = (totalSeconds) => {
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;

  return `${minutes} minutes`;
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

const Modal = ({ user, onClose, orderId }) => {
  const [isLoading, setIsLoading] = useState(true);
  const [orderData, setOrderData] = useState(null);

  useEffect(() => {
    const getOrder = async (orderId) => {
      console.log("Order ID:", orderId);
      const res = await axios
        .get(`/api/order/${orderId}`, {
          headers: {
            Authorization: `Bearer ${user.token}`, // Please ensure 'user' is defined in your component or passed as a prop
            "Content-Type": "application/json",
          },
        })
        .then((res) => {
          console.log(res);
          setOrderData(res.data);
          setIsLoading(false);
        })
        .catch((err) => {
          console.error("Error fetching data: ", err);
          alert("Error fetching data: ", err);
          setIsLoading(false);
        });
    };

    getOrder(orderId);
  }, [orderId]);

  return (
    <div
      style={{
        position: "fixed",
        top: 0,
        left: 0,
        right: 0,
        bottom: 0,
        backgroundColor: "rgba(0,0,0,0.5)",
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <div
        style={{
          backgroundColor: "white",
          padding: "20px",
          borderRadius: "8px",
        }}
      >
        {isLoading ? (
          <p>Loading...</p>
        ) : (
          <div>
            <div>{orderData.menu_name}</div>
            <div>Price: {orderData.price} yen</div>
            <div>
              expected to take{" "}
              {toMinutesAndSeconds(orderData.cook_time_seconds)}
            </div>
            <div>Ordered at: {toLocalTime(orderData.ordered_time)}</div>
            <div>Checked by: {orderData.check_staff_name}</div>
          </div>
        )}
        <br></br>
        <button onClick={onClose}>Close Modal</button>
      </div>
    </div>
  );
};

export default Modal;
