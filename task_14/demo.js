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

const order = new Order(BigInt(1), "Bratt");

order.addProduct(product1.sku, product1.price, 2, product1.inStock);
order.addProduct(product2.sku, product2.price, 1, product1.inStock);
order.addProduct(product3.sku, product3.price, 1, product1.inStock);

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

console.log(products);
