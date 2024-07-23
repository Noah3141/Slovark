import mysql.connector
class LemmatizerDb:
    from _legacy.env import config

    @classmethod
    def get_rows(cls,):
        try: 
            conn = mysql.connector.connect(**cls.config)
            cursor = conn.cursor()
            cursor.execute("SELECT DISTINCT lemma FROM russian_words;")
            rows = cursor.fetchall()

            return rows

        except mysql.connector.Error as err:
            print(f"Error: {err}")

        finally:
            # Close database connection
            if 'conn' in locals():
                conn.close()
                print("MySQL connection closed.")