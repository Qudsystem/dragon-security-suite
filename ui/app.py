"""
Protect X - Streamlit UI
Main dashboard and user interface
"""

import streamlit as st

# Page configuration
st.set_page_config(
    page_title="Protect X - Cybersecurity Defense",
    page_icon="🛡️",
    layout="wide",
    initial_sidebar_state="expanded"
)

# Custom CSS for dark cyberpunk theme
st.markdown("""
<style>
    .main {
        background-color: #0e1117;
    }
    .stApp {
        background-color: #0e1117;
    }
</style>
""", unsafe_allow_html=True)

def main():
    """Main UI entry point"""
    
    # Sidebar
    with st.sidebar:
        st.title("🛡️ Protect X")
        st.markdown("---")
        
        page = st.radio(
            "Navigation",
            ["Dashboard", "Chat", "Graph View", "Timeline", "Reports"]
        )
    
    # Main content
    if page == "Dashboard":
        show_dashboard()
    elif page == "Chat":
        show_chat()
    elif page == "Graph View":
        show_graph()
    elif page == "Timeline":
        show_timeline()
    elif page == "Reports":
        show_reports()


def show_dashboard():
    """Show main threat dashboard"""
    st.title("🛡️ Protect X - Security Dashboard")
    
    # Metrics
    col1, col2, col3, col4 = st.columns(4)
    
    with col1:
        st.metric("Active Threats", "0", "0")
    
    with col2:
        st.metric("Monitored Hosts", "0", "0")
    
    with col3:
        st.metric("Events/sec", "0", "0")
    
    with col4:
        st.metric("System Health", "100%", "0%")
    
    st.markdown("---")
    
    st.info("🚧 Dashboard under construction. Connect to Rust core and AI layer to see live data.")


def show_chat():
    """Show AI chat interface"""
    st.title("💬 AI Security Assistant")
    
    st.info("🚧 Chat interface under construction. AI layer integration pending.")


def show_graph():
    """Show network graph visualization"""
    st.title("🕸️ Attack Chain Visualization")
    
    st.info("🚧 Graph visualization under construction. Graph DB integration pending.")


def show_timeline():
    """Show forensic timeline"""
    st.title("📅 Forensic Timeline")
    
    st.info("🚧 Timeline view under construction. Event data integration pending.")


def show_reports():
    """Show reports page"""
    st.title("📄 Security Reports")
    
    st.info("🚧 Reports page under construction. Report generator integration pending.")


if __name__ == "__main__":
    main()
