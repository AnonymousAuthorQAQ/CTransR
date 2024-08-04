# echo "Running GA for atomgit c dataset"
python glm_ga.py ../dataset/atomgit/c/double_free.c ../dataset/atomgit/testc/double_free/ double_free_glm double_free_glm_success/ 
python glm_ga.py ../dataset/atomgit/c/message_queue.c ../dataset/atomgit/testc/message_queue/ message_queue_glm message_queue_glm_success/ 
python glm_ga.py ../dataset/atomgit/c/multi_thread_basic.c ../dataset/atomgit/testc/multi_thread_basic/ multi_thread_basic_glm multi_thread_basic_glm_success/
python glm_ga.py ../dataset/atomgit/c/mutex.c ../dataset/atomgit/testc/mutex/ mutex_glm mutex_glm_success/
python glm_ga.py ../dataset/atomgit/c/queue.c ../dataset/atomgit/testc/queue/ queue_glm queue_glm_success/
python glm_ga.py ../dataset/atomgit/c/queue_l.c ../dataset/atomgit/testc/queue_l/ queue_l_glm queue_l_glm_success/
python glm_ga.py ../dataset/atomgit/c/server_clients.c ../dataset/atomgit/testc/server_clients/ server_clients_glm server_clients_glm_success/
python glm_ga.py ../dataset/atomgit/c/tree.c ../dataset/atomgit/testc/tree/ tree_glm tree_glm_success/
python glm_ga.py ../dataset/atomgit/c/tree_l.c ../dataset/atomgit/testc/tree_l/ tree_l_glm tree_l_glm_success/
echo "Running GA for atomgit cpp dataset"
python glm_ga.py ../dataset/atomgit/cpp/weakptr.cpp ../dataset/atomgit/testcpp/weakptr/ weakptr_glm weakptr_glm_success/
python glm_ga.py ../dataset/atomgit/cpp/weakptr_l.cpp ../dataset/atomgit/testcpp/weakptr_l/ weakptr_l_glm weakptr_l_glm_success/
python glm_ga.py ../dataset/atomgit/cpp/derived.cpp ../dataset/atomgit/testcpp/derived/ derived_glm derived_glm_success/
python glm_ga.py ../dataset/atomgit/cpp/macro_func.cpp ../dataset/atomgit/testcpp/macro_func/ macro_func_glm macro_func_glm_success/
python glm_ga.py ../dataset/atomgit/cpp/var_func.cpp ../dataset/atomgit/testcpp/var_func/ var_func_glm var_func_glm_success/
python glm_ga.py ../dataset/atomgit/cpp/multi_thread_basic.cpp ../dataset/atomgit/testcpp/multi_thread_basic/ multi_thread_basic_cpp_glm multi_thread_basic_cpp_glm_success/
# echo "Running GA for codeforce c dataset"
python glm_ga.py ../dataset/codeforce/c/1968A.c ../dataset/codeforce/testc/1968A/ 1968A_glm 1968A_glm_success/
python glm_ga.py ../dataset/codeforce/c/1968B.c ../dataset/codeforce/testc/1968B/ 1968B_glm 1968B_glm_success/
python glm_ga.py ../dataset/codeforce/c/1968C.c ../dataset/codeforce/testc/1968C/ 1968C_glm 1968C_glm_success/
python glm_ga.py ../dataset/codeforce/c/1968D.c ../dataset/codeforce/testc/1968D/ 1968D_glm 1968D_glm_success/
python glm_ga.py ../dataset/codeforce/c/1968E.c ../dataset/codeforce/testc/1968E/ 1968E_glm 1968E_glm_success/
python glm_ga.py ../dataset/codeforce/c/1968F.c ../dataset/codeforce/testc/1968F/ 1968F_glm 1968F_glm_success/
python glm_ga.py ../dataset/codeforce/c/1968G1.c ../dataset/codeforce/testc/1968G1/ 1968G1_glm 1968G1_glm_success/
python glm_ga.py ../dataset/codeforce/c/1968G2.c ../dataset/codeforce/testc/1968G2/ 1968G2_glm 1968G2_glm_success/
# echo "Running GA for codeforce cpp dataset"
python glm_ga.py ../dataset/codeforce/cpp/1968A.cpp ../dataset/codeforce/testcpp/1968A/ 1968A_cpp_glm 1968A_cpp_glm_success/
python glm_ga.py ../dataset/codeforce/cpp/1968B.cpp ../dataset/codeforce/testcpp/1968B/ 1968B_cpp_glm 1968B_cpp_glm_success/
python glm_ga.py ../dataset/codeforce/cpp/1968C.cpp ../dataset/codeforce/testcpp/1968C/ 1968C_cpp_glm 1968C_cpp_glm_success/
python glm_ga.py ../dataset/codeforce/cpp/1968D.cpp ../dataset/codeforce/testcpp/1968D/ 1968D_cpp_glm 1968D_cpp_glm_success/
python glm_ga.py ../dataset/codeforce/cpp/1968E.cpp ../dataset/codeforce/testcpp/1968E/ 1968E_cpp_glm 1968E_cpp_glm_success/
python glm_ga.py ../dataset/codeforce/cpp/1968F.cpp ../dataset/codeforce/testcpp/1968F/ 1968F_cpp_glm 1968F_cpp_glm_success/
python glm_ga.py ../dataset/codeforce/cpp/1968G1.cpp ../dataset/codeforce/testcpp/1968G1/ 1968G1_cpp_glm 1968G1_cpp_glm_success/
python glm_ga.py ../dataset/codeforce/cpp/1968G2.cpp ../dataset/codeforce/testcpp/1968G2/ 1968G2_cpp_glm 1968G2_cpp_glm_success/

echo "mv"
# mv double_free_glm_success glm_result/success
# mv double_free_glm glm_result/result
# mv message_queue_glm_success glm_result/success
# mv message_queue_glm glm_result/result
# mv multi_thread_basic_glm_success glm_result/success
# mv multi_thread_basic_glm glm_result/result
# mv mutex_glm_success glm_result/success
# mv mutex_glm glm_result/result
# mv queue_glm_success glm_result/success
# mv queue_glm glm_result/result
# mv queue_l_glm_success glm_result/success
# mv queue_l_glm glm_result/result
# mv server_clients_glm_success glm_result/success
# mv server_clients_glm glm_result/result
# mv tree_glm_success glm_result/success
# mv tree_glm glm_result/result
# mv tree_l_glm_success glm_result/success
# mv tree_l_glm glm_result/result
# mv weakptr_glm_success glm_result/success
# mv weakptr_glm glm_result/result
# mv weakptr_l_glm_success glm_result/success
# mv weakptr_l_glm glm_result/result
# mv derived_glm_success glm_result/success
# mv derived_glm glm_result/result
# mv macro_func_glm_success glm_result/success
# mv macro_func_glm glm_result/result
# mv var_func_glm_success glm_result/success
# mv var_func_glm glm_result/result
# mv multi_thread_basic_cpp_glm_success glm_result/success
# mv multi_thread_basic_cpp_glm glm_result/result
# mv 1968A_glm_success glm_result/success
# mv 1968A_glm glm_result/result
# mv 1968B_glm_success glm_result/success
# mv 1968B_glm glm_result/result
# mv 1968C_glm_success glm_result/success
# mv 1968C_glm glm_result/result
# mv 1968D_glm_success glm_result/success
# mv 1968D_glm glm_result/result
# mv 1968E_glm_success glm_result/success
# mv 1968E_glm glm_result/result
# mv 1968F_glm_success glm_result/success
# mv 1968F_glm glm_result/result
# mv 1968G1_glm_success glm_result/success
# mv 1968G1_glm glm_result/result
# mv 1968G2_glm_success glm_result/success
# mv 1968G2_glm glm_result/result
# mv 1968A_cpp_glm_success glm_result/success
# mv 1968A_cpp_glm glm_result/result
# mv 1968B_cpp_glm_success glm_result/success
# mv 1968B_cpp_glm glm_result/result
# mv 1968C_cpp_glm_success glm_result/success
# mv 1968C_cpp_glm glm_result/result
# mv 1968D_cpp_glm_success glm_result/success
# mv 1968D_cpp_glm glm_result/result
# mv 1968E_cpp_glm_success glm_result/success
# mv 1968E_cpp_glm glm_result/result
# mv 1968F_cpp_glm_success glm_result/success
# mv 1968F_cpp_glm glm_result/result
# mv 1968G1_cpp_glm_success glm_result/success
# mv 1968G1_cpp_glm glm_result/result
# mv 1968G2_cpp_glm_success glm_result/success
# mv 1968G2_cpp_glm glm_result/result
echo "Done"
