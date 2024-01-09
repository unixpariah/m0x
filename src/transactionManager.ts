import "./styles.css";
import TransactionManager from "./TransactionManager.svelte";
const app = new TransactionManager({
  target: document.getElementById("transactionManager"),
});

export default transactionManager;
