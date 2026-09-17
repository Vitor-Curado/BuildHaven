import Dropdown from "./Dropdown";
import ThemeSwitcher from "./ThemeSwitcher";

interface NavItem {
  href: string;
  icon: string;
  title: string;
}

interface Doc {
  slug: string;
  title: string;
}

const Navbar = ({ navItems, docs }: { navItems: NavItem[]; docs: Doc[] }) => {
  return (
    <nav className="navbar">
      <ul className="navbar-group navbar-left">
        {navItems.map((item) => (
          <li key={item.href}>
            <a href={item.href} className="navbar-link">
              <img
                src={`/media/icons/${item.icon}`}
                alt={item.title}
                className="navbar-icon"
              />
              {item.title}
            </a>
          </li>
        ))}

        <Dropdown buttonContent="Docs">
          <ul className="dropdown-menu">
            {docs.map((doc) => (
              <li key={doc.slug}>
                <a href={`/docs/${doc.slug}`} className="dropdown-link">
                  {" "}
                  {doc.title}{" "}
                </a>
              </li>
            ))}
          </ul>
        </Dropdown>
      </ul>

      <ul className="navbar-group navbar-right">
        <li>
          <a href="/login" className="navbar-link util-icon">
            <img
              src="/media/icons/login.svg"
              alt="login"
              className="navbar-icon"
            />
          </a>
        </li>

        <Dropdown
          buttonContent={
            <img
              src="/media/icons/theme.svg"
              alt="Theme"
              className="navbar-icon"
            />
          }
        >
          <ThemeSwitcher />
        </Dropdown>
      </ul>
    </nav>
  );
};

export default Navbar;
