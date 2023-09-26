import { useAuth } from "../context/authContext";
import { useState } from "react";
import { useRouter } from 'next/router';
import styles from '@/styles/Login.module.css'

const LoginPage = () => {
    const router = useRouter();
    const [name, setName] = useState('');
    const [password, setPassword] = useState('');
    const [isLoggingIn, setIsLoggingIn] = useState(false);
    const { login } = useAuth();
    const handleNameChange = (e) => {
        setName(e.target.value);
    };
    const handlePasswordChange = (e) => {
        setPassword(e.target.value);
    };
    const handleSubmit = async (e) => {
      e.preventDefault();
      setIsLoggingIn(true);
      let isSuccess = await login(name, password);
      setIsLoggingIn(false);
      if (isSuccess) {
        router.push('/');
      } else {
        alert('Login failed.');
      }
    };

    return (
        <div>
            <form id={`${styles.loginForm}`} onSubmit={handleSubmit} method="post">
                <div className={`${styles.container}`}>
                    <label htmlFor="name"><b>Username</b></label>
                    <input
                        id="name"
                        type="text"
                        className={`${styles.formInput}`}
                        placeholder="Enter Username"
                        name="name"
                        onChange={handleNameChange}
                        required
                    />
                    <label htmlFor="password"><b>Password</b></label>
                    <input
                        id="password"
                        type="password"
                        className={`${styles.formInput}`}
                        placeholder="Enter Password"
                        name="password"
                        onChange={handlePasswordChange}
                        disabled={isLoggingIn}
                        required
                    />
                    <button className={`${styles.formButton}`} type="submit">Login</button>
                </div>
            </form>
        </div>
    );
};

export default LoginPage;