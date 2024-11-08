import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './main.css';

import eyeShow from './assets/eye-show.png';
import eyeHide from './assets/eye-hide.png';

function AppPassword() {
    const [message, setMessage] = useState(null);
    const [messageColor, setMessageColor] = useState('red'); // Default color for errors
    const [toggleView, setToggleView] = useState("hide");
    const [passwordType, setPasswordType] = useState("password");
    const [password, setPassword] = useState(null); // Store the hashed password from the database
    const [isPassword, setIsPassword] = useState(false);

    // Function to retrieve the hashed password from the database
    const getPassword = async () => {
        try {
            const result = await invoke('get_password_hash');
            setPassword(result || null); // Set the retrieved hashed password or null if no password
            setIsPassword(true);
        } catch (error) {
            console.error("Error fetching password from the database:", error);
            setPassword(null); // Set to null in case of error
            setIsPassword(false);
        }
    };

    // Call getPassword on component mount
    useEffect(() => {
        getPassword(); // Fetch password from the database when the component loads
    }, []);

    const handleToggleView = () => {
        if (toggleView === "hide") {
            setToggleView("show");
            setPasswordType("text");  // Show password as plain text
        } else {
            setToggleView("hide");
            setPasswordType("password");  // Hide password
        }
    };

    const handlePasswordSubmit = async () => {
        const currentPassword = document.querySelector('.previous-password')?.value;
        const newPassword = document.querySelector('.new-password').value;
        const confirmPassword = document.querySelector('.confirm-password').value;

        // Error 0: Check if new password is empty
        if (!newPassword) {
            setMessage("new password cannot be empty.".toUpperCase());
            setMessageColor('red');
            return;
        }

        // Skip verification if no password is set (password is null)
        if (password !== null) {
            // Error: Check if current password is wrong
            try {
                const isPasswordCorrect = await invoke('verify_password', { hash: password, password: currentPassword });
                if (!isPasswordCorrect) {
                    setMessage("current password is wrong.".toUpperCase());
                    setMessageColor('red');
                    return; // Prevent further actions if current password is incorrect
                }
            } catch (error) {
                setMessage("error verifying current password.".toUpperCase());
                setMessageColor('red');
                return; // Prevent further actions if there's an error in verification
            }
        }

        // Error 1: Check if new password and confirm password do not match
        if (newPassword !== confirmPassword) {
            setMessage("passwords do not match.".toUpperCase());
            setMessageColor('red');
            return;
        }

        // Error 2: Check if new password is the same as old password
        if (currentPassword === newPassword) {
            setMessage("new password cannot be the same as the current password.".toUpperCase());
            setMessageColor('red');
            return;
        }

        // Hash the new password
        try {
            const hashedPassword = await invoke('hash_password', { password: newPassword });

            // Set the new hashed password in the database
            await invoke('set_password_hash', { newHash: hashedPassword }); // Use new_hash

            // Success: Password has been set
            setMessage("password has been set. please close this window.".toUpperCase());
            setMessageColor('green');
        } catch (error) {
            setMessage("error setting new password.".toUpperCase());
            setMessageColor('red');
        }
    };

    return (
        <div className='create-password-body'>
            {isPassword && <input className='previous-password' type={passwordType} placeholder='Current Password' />}
            <input className='new-password' type={passwordType} placeholder='New Password' />
            <input className='confirm-password' type={passwordType} placeholder='Confirm Password' />
            <div className='create-password-button-container'>
                <button className='set-password-button' onClick={handlePasswordSubmit}>Confirm</button>
                <button className='toggle-password-view' onClick={handleToggleView}>
                    <img src={toggleView === "hide" ? eyeHide : eyeShow} alt="toggle view" />
                </button>
            </div>

            {message && <p className='error-message' style={{ color: messageColor }}>{message}</p>}
        </div>
    );
}

export default AppPassword;
