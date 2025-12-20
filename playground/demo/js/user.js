export class User {
  constructor(name, age) {
    this.name = name;
  }

  sayHello() {
    console.log(`Hello, ${this.name}!`);
  }
}