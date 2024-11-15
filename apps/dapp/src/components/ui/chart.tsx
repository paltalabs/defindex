import React, { useState, useEffect } from 'react';
import { Doughnut } from 'react-chartjs-2';
const Dashboard = () => {
  const [data, setData] = useState({
    labels: ['a', 'b',],
    datasets: [
      {
        label: 'Real-time Data',
        data: [1, 2,],
        borderColor: 'rgba(75, 192, 192, 1)',
        backgroundColor: '[rgba(75, 192, 192, 0.5)]',
      },
    ],
  });

  return (
    <div className="dashboard">
      <Doughnut data={data} />
    </div>
  );
};
export default Dashboard;