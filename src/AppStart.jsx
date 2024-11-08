import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './main.css';

import enterIcon from './assets/enter-icon.png';

function AppStart() {
    const [password, setPassword] = useState('');
    const [result, setResult] = useState(null);

    const handleLogin = async () => {
        try {
            // Store the hash locally instead of using setHash immediately
            const fetchedHash = await invoke('get_password_hash');

            // Now use the locally fetched hash for password verification
            const isPasswordCorrect = await invoke('verify_password', { hash: fetchedHash, password: password });

            if (!isPasswordCorrect) {
                console.error("Password is incorrect.");
                setResult(false);  // Indicate failure
                return;
            } else {
                setResult(true);  // Indicate success, class change will be handled by useEffect
            }

        } catch (error) {
            console.error("Error checking password:", error);
            setResult(false);  // Indicate failure if an error occurs
        }
    };

    const handleCreatePasswordClick = async () => {
        try {
            if (!(await invoke('is_password_open'))) {
                await invoke('create_password_window');
            }
        } catch (error) {
            console.error("Error showing create password window:", error);
        }
    };

    // useEffect to handle the class change and window invocation when result is true
    useEffect(() => {
        if (result === true) {
            const timer = setTimeout(async () => {
                await invoke('create_app_window');
            }, 1000); // Wait for 2 seconds before invoking 'create_app_window'

            // Cleanup function to clear the timeout if the component is unmounted
            return () => clearTimeout(timer);
        }
    }, [result]);  // Runs this effect whenever 'result' changes

    return (
        <div className='startup-body'>
            <div className={`input-container ${result === true ? 'input-correct' : result === false ? 'input-incorrect' : ''}`}>
                <input
                    className='startup-input'
                    type='password'
                    onChange={(e) => setPassword(e.target.value)}
                    placeholder='Enter Password'
                />
            </div>

            <button className='startup-button' onClick={handleLogin}>
                <div className='button-content'>
                    <img className='startup-button-img' src={enterIcon} alt='enter'/>
                    <span className="startup-button-text">Enter</span>
                </div>
            </button>

            <p className='create-password' onClick={handleCreatePasswordClick}>Create Password</p>
        </div>
    );
}

export default AppStart;
