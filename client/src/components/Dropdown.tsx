import { useEffect, useRef, useState } from "react";
import type { ReactNode } from "react";

interface DropdownProps {
  buttonContent: ReactNode;
  children: ReactNode;
}

const Dropdown = ({ buttonContent, children }: DropdownProps) => {
  const [open, setOpen] = useState(false);
  const dropdownRef = useRef<HTMLLIElement>(null);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        dropdownRef.current &&
        !dropdownRef.current.contains(event.target as Node)
      ) {
        setOpen(false);
      }
    };

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        setOpen(false);
      }
    };

    document.addEventListener("click", handleClickOutside);
    document.addEventListener("keydown", handleKeyDown);

    return () => {
      document.removeEventListener("click", handleClickOutside);
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, []);

  return (
    <li ref={dropdownRef} className={`dropdown${open ? " open" : ""}`}>
      <button
        type="button"
        className="dropdown-button"
        onClick={(event) => {
          event.stopPropagation();
          setOpen((current) => !current);
        }}
        aria-expanded={open}
      >
        {buttonContent}
      </button>

      {children}
    </li>
  );
};

export default Dropdown;
