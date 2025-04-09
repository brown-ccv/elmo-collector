SELECT setval('oscar.cpu_id_seq', (SELECT MAX(id) FROM oscar.cpu), true);

SELECT setval('oscar.gpu_id_seq', (SELECT MAX(id) FROM oscar.gpu), true);


SELECT setval('oscar.power_save_id_seq', (SELECT MAX(id) FROM oscar.power_save), true);

SELECT setval('oscar.power_save_cpu_id_seq', (SELECT MAX(id) FROM oscar.power_save_cpu), true);

SELECT setval('oscar.power_save_gpu_id_seq', (SELECT MAX(id) FROM oscar.power_save_gpu), true);

