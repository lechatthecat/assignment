import { useAuth } from "../context/authContext";

const LoginPage = () => {
    const { login } = useAuth();
    return (
        <div>
            <form action="action_page.php" method="post">
                <div class="imgcontainer">
                    <img src="img_avatar2.png" alt="Avatar" class="avatar" />
                </div>

                <div class="container">
                    <label for="uname"><b>Username</b></label>
                    <input type="text" placeholder="Enter Username" name="uname" required />

                    <label for="psw"><b>Password</b></label>
                    <input type="password" placeholder="Enter Password" name="psw" required />

                    <button type="submit">Login</button>
                </div>
            </form>
        </div>
    );
};

export default LoginPage;