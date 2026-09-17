import "./App.css";
import Navbar from "./components/Navbar";

const navItems = [
    {
        title: "Home",
        href: "/",
        icon: "Home",
    },
    {
        title: "Blog",
        href: "/blog",
        icon: "Blog",
    },
    {
        title: "Resume",
        href: "/resume",
        icon: "Resume",
    },
    {
        title: "Contact",
        href: "/contact",
        icon: "Contact",
    },
];

const docs = [
    {
        title: "Decisions",
        slug: "decicions",
    },
    {
        title: "Learned concepts",
        slug: "learned-concepts",
    },
    {
        title: "Learning journey",
        slug: "learning-journey",
    },
    {
        title: "Scope",
        slug: "scope",
    },
    {
        title: "What is software engineering",
        slug: "what-is-software-engineering",
    },
];

function App() {
  return (
    <>
      <Navbar navItems={navItems} docs={docs}/>

      <main className="app">
        <h1> BuildHaven Admin </h1>
        <p> React + TypeScript + Vite is working. </p>
      </main>
    </>
  );
}

export default App;
