// contexts/authContext.js
import { createContext, useContext, useState, useEffect } from "react";

const AuthContext = createContext();

export const AuthProvider = ({ children }) => {
  const [user, setUser] = useState(null);
  const [loading, setLoading] = useState(true);

  const login = async (name, password) => {
    try {
      const response = await fetch('/api/auth/login', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          name: name,
          password: password,
        }),
      });

      if (!response.ok) {
        // Handle login failure
        console.error('Login failed');
        return false;
      }

      const user_data = await response.json();
      user_data.sub = user_data.name;
      console.log(user_data);
      // Store the token in localStorage
      window.localStorage.setItem('login_token', user_data.token);
      // Set the user information in your application state
      setUser(user_data);
      return true;
    } catch (error) {
      console.error('Login error:', error);
      return false;
    }
  };

  const getCurrentUser = async () => {
    try {
      const token = window.localStorage.getItem('login_token');
      if (token === '' || token === 'undefined') {
        console.log(token);
        return false;
      }
      const response = await fetch('/api/auth/current_user', {
        method: 'GET',
        headers: {
          'Authorization': `Bearer ${token}`,
        },
      });

      if (response.ok) {
        const user = await response.json();
        user.token = token;
        return user;
      } else {
        return false;
      }
    } catch (error) {
      console.error("Failed to fetch current user", error);
      return false;
    }
  };

  useEffect(() => {
    const loadUser = async () => {
      try {
        let user = await getCurrentUser();
        if (user) {
          setUser(user);
        } else {
          console.log('Failed to load user');
          setUser(null);
        }
      } catch (error) {
        console.error("Failed to fetch current user", error);
        setUser(null);
      } finally {
        setLoading(false);
      }
    }

    loadUser();
  }, [setUser, setLoading]);

  const logout = async () => {
    try {
      localStorage.removeItem('login_token');
      // Handle logout logic here
      setUser(null);
      return true; // logout successful
    } catch (error) {
      // Handle logout failure
      return false; // logout fail
    }
  };

  return (
    <AuthContext.Provider value={{ user, setUser, loading, login, logout }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};
