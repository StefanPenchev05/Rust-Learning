import React, { ReactNode } from 'react'
import NavBar from '../NavBar'
import Footer from '../Footer'

interface LayoutProps {
    children: ReactNode
}

const Layout:React.FC<LayoutProps> = ({children}) => {
  return (
    <div className='flex flex-col min-h-svh'>
        <header>
            <NavBar/>
        </header>
        <main className='flex justify-center items-center flex-grow-5'>
            {children}
        </main>

        <footer className='flex-grow bg-gray-700'>
            <Footer/>
        </footer>
    </div>
  )
}

export default Layout