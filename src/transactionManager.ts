import "./styles.css";
import TransactionManager from "./TransactionManager.svelte";

const app = new TransactionManager({
  target: document.getElementById("app"),
});

export default app;
