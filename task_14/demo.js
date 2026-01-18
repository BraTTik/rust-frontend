import { Order } from "./pkg/task_14.js";
import { ProductView } from "./product-view.js";

const product1 = {
  sku: "TLOG-Ai-001",
  price: 30.99,
  inStock: true,
}

const product2 = {
  sku: "TLOG-002",
  price: 10.99,
  inStock: false,
}

const product3 = {
  sku: "TLOG-003",
  price: 20.99,
  inStock: true,
}

const createProduct = (i) => ({
  sku: `TLOG-${String(i + 1).padStart(3, "0")}`,
  price: Number((Math.random() * 100).toFixed(2)),
  quantity: 3,
  inStock: Math.random() > 0.5,
})

const order = new Order(BigInt(1), "Bratt");

for (let i = 0; i < 150000; i++) {
  const p = createProduct(i)
  order.addProduct(p.sku, p.price, p.quantity, p.inStock);
}

console.time("JSON products");
order.getItemsJS();
console.timeEnd("JSON products");

console.time("Bin products");
let p = readBinProducts();
console.timeEnd("Bin products");

function readBinProducts() {
  const order_bin = order.getItemsBin();
  const arrayLength = order_bin.length;
  let offset = 0;
  let products = [];

  while (offset < arrayLength) {
    const view = new ProductView(order_bin, offset);
    products.push({
      sku: view.sku,
      price: view.price,
      quantity: view.quantity,
      inStock: view.inStock
    });
    offset += view.length;
  }
  console.log("Bin products finished reading");
  return products;
}
