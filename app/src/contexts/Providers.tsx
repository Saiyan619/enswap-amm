import { ClusterProvider } from './ClusterProvider'
import ReactQueryProvider from './ReactQueryProvider'
import { WalletAdapterProvider } from './WalletProvider'


export const Providers = ({ children }: { children: React.ReactNode }) => {
  return (
    <ReactQueryProvider>
      <ClusterProvider>
      <WalletAdapterProvider>{children}</WalletAdapterProvider>
      </ClusterProvider>
    </ReactQueryProvider>
  )
}
