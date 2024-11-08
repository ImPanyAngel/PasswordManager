import { invoke } from '@tauri-apps/api/core';
import { useState } from 'react';
import { Bounce, ToastContainer, toast } from 'react-toastify';
import './components.css';

import eyeShow from '../assets/eye-show.png';
import eyeHide from '../assets/eye-hide.png';
import copyIcon from '../assets/copy-icon.png';
import deleteIcon from '../assets/delete-icon.png';

function Password({ passwordID, plainText, refreshPasswords }) {
    const [toggleView, setToggleView] = useState("hide");
    const [passwordType, setPasswordType] = useState("password");

    const handleToggleView = () => {
        setToggleView((prevView) => (prevView === "hide" ? "show" : "hide"));
        setPasswordType((prevType) => (prevType === "password" ? "text" : "password"));
    };

    const copyToClipboard = async () => {
        try {
            await invoke('copy_to_clipboard', { text: plainText });
            toast("Copied to Clipboard");
        } catch (error) {
            console.error("Failed to copy password:", error);
            toast("Failed to copy password");
        }
    };

    const handleDeletePassword = async () => {
        try {
            await invoke('delete_password', { passwordId: passwordID });
            refreshPasswords(); // Trigger a refresh after deletion
        } catch (error) {
            console.error("Failed to delete password:", error);
        }
    };

    return (
        <>
            <div className="password-item">
                <div className="password">{passwordType === "password" ? "••••••••••••••" : plainText}</div>
                <div className='password-list-button-container'>
                    <button className='password-list-toggle-view' onClick={handleToggleView}>
                        <img className='password-list-toggle-view-img' src={toggleView === "hide" ? eyeHide : eyeShow} alt="toggle view" />
                    </button>
                    <button className='password-list-delete' onClick={handleDeletePassword}>
                        <img className='password-list-delete-img' src={deleteIcon} alt='delete'/>
                    </button>
                    <button className='password-list-clipboard-copy' onClick={copyToClipboard}>
                        <img className='password-list-clipboard-copy-img' src={copyIcon} alt='copy' />
                    </button>
                </div>
            </div>
            <ToastContainer
                position='bottom-center'
                autoClose={1000}
                hideProgressBar={true}
                closeOnClick
                rtl={false}
                theme="dark"
                transition={Bounce}
                closeButton={false}
                limit={1}
            />
        </>

    );
}

export default Password;
