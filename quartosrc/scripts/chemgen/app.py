import streamlit as st
st.set_page_config(layout='wide')

def process_pattern(pattern):
    return pattern

with st.form("my_form"):
    col1, col2 = st.columns(2)
    with col1:
        st.number_input("Number of species", key="num_species", min_value=1, max_value=256, value=1, help="Number of atom species present in the world (between 1 and 256)")
    with col2:
        st.number_input("Number of states", key="num_states", min_value=1, max_value=256, value=2, help="Number of states an atom can take (between 1 and 256)")

    pattern = st.text_area("Chemistry pattern", key="chempattern", height=100, help="Write a chemistry pattern here to use for generating a chemistry.cfg")

    submitted = st.form_submit_button("Generate chemistry")

    output_area = st.markdown(body="**Generated chemistry**\n```\n```")

    if submitted:
        generated_chemistry = process_pattern(pattern)
        st.session_state['output'] = generated_chemistry
        output_area.empty()
        output_area = st.markdown(body="**Generated chemistry**\n```\n"+generated_chemistry+"\n```")

if 'output' in st.session_state:
    st.download_button(label="Download generated chemistry", data=st.session_state['output'], file_name="chemistry.cfg")

hide_streamlit_style = """
<style>
#MainMenu {visibility: hidden;}
footer {visibility: hidden;}
.stAppDeployButton {display:none;}
.stAppHeader {display:none;}
</style>

"""
st.markdown(hide_streamlit_style, unsafe_allow_html=True) 

margins_css = """
 <style>
               .block-container {
                    padding-top: 0rem;
                    padding-bottom: 0rem;
                    padding-left: 0rem;
                    padding-right: 0rem;
                }
        </style>
"""

st.markdown(margins_css, unsafe_allow_html=True)

