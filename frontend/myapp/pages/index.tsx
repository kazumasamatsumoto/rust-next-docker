import type { NextPage } from "next";
import { Axios } from "../lib/api";

const Home: NextPage = () => {
  Axios.post(`api/proxy/users`, {
    id: 1,
    username: "test",
  })
    .then((res) => {
      console.log(res);
    })
    .catch((error) => {
      console.log(error);
    });
  return <h1 className="text-3xl font-bold underline">Hello world!</h1>;
};

export default Home;
