import { useEffect } from 'react';
import { useRouter } from 'next/router';
import { useAuth } from "../context/authContext";

const ProtectedPage = () => {
    const { isAuthenticated } = useAuth();
    const router = useRouter();

    useEffect(() => {
        if (!isAuthenticated) {
            router.push('/login'); // redirecting to the login page
        }
    }, [isAuthenticated, router]);

    return (
        <div>
            {/* Your protected content */}
        </div>
    );
};

export default ProtectedPage;
