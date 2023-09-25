import React, { useEffect, useState } from 'react';

const Countdown = ({ targetTimeString }) => {
  const [timeRemaining, setTimeRemaining] = useState(null);

  useEffect(() => {
    const intervalId = setInterval(() => {
      const now = new Date();
      const target = new Date(targetTimeString);
      const remainTime = target - now;

      if (remainTime <= 0) {
        clearInterval(intervalId);
        setTimeRemaining({ minutes: 0, seconds: 0, finished: true });
      } else {
        const minutes = Math.floor(remainTime / 1000 / 60) % 60;
        const seconds = Math.floor(remainTime / 1000) % 60;
        setTimeRemaining({ minutes, seconds, finished: false });
      }
    }, 1000);

    return () => clearInterval(intervalId);
  }, [targetTimeString]);

  if (timeRemaining === null) return <div>Loading...</div>;

  if (timeRemaining.finished) return <div>Finished</div>;

  return (
    <div>
      {timeRemaining.minutes}min {timeRemaining.seconds}sec
    </div>
  );
};

export default Countdown;